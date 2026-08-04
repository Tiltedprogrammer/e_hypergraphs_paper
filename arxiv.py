"""Flatten a LaTeX paper and its dependencies into a self-contained,
arXiv-ready directory.

Usage:
    python arxiv.py <main.tex> <out_dir>

It walks the main file (recursing through \\input/\\include), copies every
referenced figure, included file, bibliography and *local* style package into
<out_dir>, rewriting the references to match the flattened layout. Paths are
resolved relative to the main file's directory (the LaTeX compilation root),
mirroring how LaTeX itself resolves them.
"""

import re
import shutil
import sys
from pathlib import Path

from pylatexenc.latexwalker import LatexWalker, LatexMacroNode, LatexSpecialsNode
from pylatexenc.latex2text import LatexNodes2Text

FIGURE_EXTENSIONS = ['.pdf', '.png', '.jpg', '.jpeg', '.eps', '.tikz']
# Order matters: try no-extension first, then figures, then source/style files.
RESOLVE_EXTENSIONS = [''] + FIGURE_EXTENSIONS + ['.tex', '.bib', '.sty',
                                                 '.tikzstyles', '.tikzdefs']

FIGURE_MACROS = {'includegraphics', 'tikzfig'}
INPUT_MACROS = {'input', 'include', 'subfile'}
BIB_MACROS = {'bibliography', 'addbibresource'}
PACKAGE_MACROS = {'usepackage', 'RequirePackage'}
HANDLED_MACROS = FIGURE_MACROS | INPUT_MACROS | BIB_MACROS | PACKAGE_MACROS


# --------------------------------------------------------------------------- #
# Parsing: extract (macroname, argument) pairs for every dependency reference. #
# --------------------------------------------------------------------------- #

def _node_text(node):
    """Render a group/argument node back to its plain-text content."""
    if node is None:
        return None
    nodelist = getattr(node, 'nodelist', None)
    if nodelist is not None:
        return LatexNodes2Text().nodelist_to_text(nodelist).strip()
    return LatexNodes2Text().nodelist_to_text([node]).strip()


def _mandatory_arg(argd):
    """Return the text of the last *mandatory* ({...}) argument, if any."""
    if argd is None or not argd.argnlist:
        return None
    for arg in reversed(argd.argnlist):
        if arg is None:
            continue
        delimiters = getattr(arg, 'delimiters', None)
        if delimiters and delimiters[0] == '[':  # optional [..] argument
            continue
        return _node_text(arg)
    return None


def extract_macros(nodes):
    macros = []
    nodes_iter = iter(nodes)
    for node in nodes_iter:
        if isinstance(node, LatexSpecialsNode):
            continue
        if isinstance(node, LatexMacroNode) and node.macroname in HANDLED_MACROS:
            arg = _mandatory_arg(getattr(node, 'nodeargd', None))
            if arg is None:
                # Custom macros (e.g. \tikzfig) aren't in pylatexenc's macro db,
                # so their {...} argument is the next group node in the stream.
                arg = _node_text(next(nodes_iter, None))
            if arg:
                macros.append((node.macroname, arg))
            continue
        # Recurse into environments / groups and into any structured arguments.
        if getattr(node, 'nodelist', None) is not None:
            macros.extend(extract_macros(node.nodelist))
        argd = getattr(node, 'nodeargd', None)
        if argd is not None:
            for arg in argd.argnlist:
                if getattr(arg, 'nodelist', None) is not None:
                    macros.extend(extract_macros(arg.nodelist))
    return macros


def find_macros_recursively(latex_code):
    nodelist, _, _ = LatexWalker(latex_code).get_latex_nodes()
    return extract_macros(nodelist)


# --------------------------------------------------------------------------- #
# Packaging.                                                                   #
# --------------------------------------------------------------------------- #

class Packager:
    def __init__(self, root_dir, out_dir):
        # root_dir is the compilation directory: all references resolve here.
        self.root_dir = Path(root_dir).resolve()
        self.out_dir = Path(out_dir).resolve()
        self.out_dir.mkdir(parents=True, exist_ok=True)
        self.targets = {}          # resolved source Path -> output relative Path
        self.used = {}             # output relative path str -> source Path
        self.processed = set()     # tex files already flattened

    def resolve(self, ref, extensions=RESOLVE_EXTENSIONS):
        for ext in extensions:
            candidate = (self.root_dir / (ref + ext)).resolve()
            if candidate.is_file():
                return candidate
        return None

    def target_for(self, src):
        """Pick a collision-free output path for a source file and copy it."""
        if src in self.targets:
            return self.targets[src]

        base = Path('figures') if src.suffix in FIGURE_EXTENSIONS else Path('.')
        name = src.name
        candidate = base / name
        counter = 1
        while str(candidate) in self.used and self.used[str(candidate)] != src:
            # Disambiguate same-named files from different source folders.
            name = f"{src.parent.name}_{src.name}" if counter == 1 \
                else f"{src.parent.name}_{counter}_{src.name}"
            candidate = base / name
            counter += 1

        dest = self.out_dir / candidate
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(src, dest)
        self.used[str(candidate)] = src
        self.targets[src] = candidate
        return candidate

    @staticmethod
    def new_reference(macro, src, target):
        """The rewritten reference string for the flattened layout."""
        if macro in FIGURE_MACROS:
            # tikzfig wants no extension; includegraphics keeps it.
            return target.with_suffix('').as_posix() if src.suffix == '.tikz' \
                else target.as_posix()
        if macro in INPUT_MACROS:
            return target.with_suffix('').as_posix() if src.suffix == '.tex' \
                else target.as_posix()
        if macro == 'bibliography':       # \bibliography{foo}  (no .bib)
            return target.with_suffix('').as_posix()
        if macro == 'addbibresource':     # \addbibresource{foo.bib} (keep .bib)
            return target.as_posix()
        return None  # package macros: file is copied but the reference is unchanged

    def process(self, tex_path):
        tex_path = Path(tex_path).resolve()
        if tex_path in self.processed:
            return
        self.processed.add(tex_path)

        content = tex_path.read_text(encoding='utf-8')
        replacements = []

        for macro, ref in find_macros_recursively(content):
            if macro in PACKAGE_MACROS:
                # Copy only *local* .sty files; leave system packages alone.
                for pkg in (p.strip() for p in ref.split(',')):
                    if not pkg:
                        continue
                    src = self.resolve(pkg, extensions=['.sty'])
                    if src is not None:
                        self.target_for(src)
                continue

            src = self.resolve(ref)
            if src is None:
                print(f"Warning: could not resolve \\{macro}{{{ref}}}")
                continue

            target = self.target_for(src)
            new_ref = self.new_reference(macro, src, target)
            if new_ref and new_ref != ref:
                replacements.append((ref, new_ref))

            if macro in INPUT_MACROS and src.suffix == '.tex':
                self.process(src)

        updated = content
        for old, new in replacements:
            # Brace-scoped replacement avoids clobbering unrelated substrings.
            updated = re.sub(r'\{\s*' + re.escape(old) + r'\s*\}',
                             '{' + new + '}', updated)

        (self.out_dir / tex_path.name).write_text(updated, encoding='utf-8')
        print(f"Processed {tex_path.name}")


def main(main_tex_path, out_dir):
    main_tex = Path(main_tex_path).resolve()
    packager = Packager(main_tex.parent, out_dir)
    packager.process(main_tex)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        sys.exit("Usage: python arxiv.py <main.tex> <out_dir>")
    main(sys.argv[1], sys.argv[2])
