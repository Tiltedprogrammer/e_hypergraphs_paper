import shutil
from pathlib import Path
from pylatexenc.latexwalker import LatexWalker, LatexEnvironmentNode, LatexMacroNode
from pylatexenc.latex2text import LatexNodes2Text
import sys

FIGURE_EXTENSIONS = ['.pdf', '.png', '.jpg', '.jpeg', '.eps', '.tikz']
FIGURE_MACROS = {'includegraphics', 'tikzfig'}
INPUT_MACROS = {'input', 'include'}
BIB_MACROS = {'bibliography'}

def find_macros_recursively(latex_code):
    walker = LatexWalker(latex_code)
    nodelist, pos, len_ = walker.get_latex_nodes()
    return extract_macros(nodelist)

def extract_macros(nodes):
    macros = []
    nodes_iter = iter(nodes)
    while True:
        try:
            node = next(nodes_iter)
            if isinstance(node, LatexMacroNode):
                if node.macroname in FIGURE_MACROS | INPUT_MACROS | BIB_MACROS:
                    if node.macroname == r'tikzfig':
                        node = next(nodes_iter)
                        macros.append((r'tikzfig', LatexNodes2Text().nodelist_to_text(node.nodelist)))
                    else:                        
                        arg = node.nodeargd.argnlist[0]
                        if arg is not None:
                            macros.append((node.macroname, LatexNodes2Text().nodelist_to_text(arg.nodelist)))
            elif hasattr(node, 'nodelist'):
                macros.extend(extract_macros(node.nodelist))
            elif hasattr(node, 'nodeargd'):
                for arg in node.nodeargd.argnlist:
                    if hasattr(arg, 'nodelist'):
                        macros.extend(extract_macros(arg.nodelist))
        except StopIteration:
            break
    # for node in nodes:
    #     if isinstance(node, LatexMacroNode):
    #         if node.macroname in FIGURE_MACROS | INPUT_MACROS | BIB_MACROS:
    #             arg = node.nodeargd.argnlist[0]
    #             if arg is not None:
    #                 macros.append((node.macroname, LatexNodes2Text().nodelist_to_text(arg.nodelist)))
    #     elif hasattr(node, 'nodelist'):
    #         macros.extend(extract_macros(node.nodelist))
    #     elif hasattr(node, 'nodeargd'):
    #         for arg in node.nodeargd.argnlist:
    #             if hasattr(arg, 'nodelist'):
    #                 macros.extend(extract_macros(arg.nodelist))
    return macros

def resolve_and_copy(dep_path, base_dir, out_dir):
    base_dir = Path(base_dir)
    out_dir = Path(out_dir)

    stem = dep_path
    found = None

    # Try resolving with extensions
    for ext in [''] + FIGURE_EXTENSIONS + ['.tex', '.bib']:
        p = (Path(base_dir, (stem + ext))).resolve()
        if p.exists():
            found = p
            break

    if found is None:
        print(f"Warning: Could not resolve {dep_path}")
        return None, None

    if found.suffix in FIGURE_EXTENSIONS:
        rel_target = Path('figures') / found.name
        dest_path = out_dir / rel_target
        dest_path.parent.mkdir(parents=True, exist_ok=True)
    else:
        rel_target = found.name
        dest_path = out_dir / rel_target

    shutil.copy(found, dest_path)
    if found.suffix == '.tikz':
        dest_path = dest_path.parent / dest_path.stem
    return found, dest_path.relative_to(out_dir)

def process_main_tex(main_tex_path, out_dir):
    main_tex_path = Path(main_tex_path).resolve()
    out_dir = Path(out_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    content = main_tex_path.read_text(encoding='utf-8')
    macros = find_macros_recursively(content)

    updated_content = content
    replacements = {}

    for cmd, arg in macros:
        orig = arg
        _, new_rel_path = resolve_and_copy(arg, main_tex_path.parent, out_dir)
        if new_rel_path:
            # Store replacements for post-processing
            replacements[orig] = str(new_rel_path)

            # If it's a .tex input/include, recurse
            if cmd in INPUT_MACROS:
                full_path = main_tex_path.parent / (arg if arg.endswith('.tex') else arg + '.tex')
                if full_path.exists():
                    process_main_tex(full_path, out_dir)

    # Replace occurrences in text
    for old, new in replacements.items():
        updated_content = updated_content.replace(old, new)

    out_main_path = out_dir / main_tex_path.name
    out_main_path.write_text(updated_content, encoding='utf-8')
    print(f"Processed {main_tex_path.name}")

if __name__ == '__main__':
    process_main_tex(sys.argv[1], sys.argv[2])