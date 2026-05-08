MAIN=CategoricalEgraphsForLambdaCalculi
LATEXMK=latexmk
LATEXFLAGS=-pdf -shell-escape -synctex=1 -interaction=nonstopmode -output-directory=build

FIGS := $(shell find ./figures -type f -name "*.tikz")
PDFS := $(patsubst ./%.tikz,cache/%.pdf,$(FIGS))

.PHONY: all pdf figures clean cleanfig

all: pdf

pdf: figures
	$(LATEXMK) $(LATEXFLAGS) $(MAIN).tex

figures: $(PDFS)

cache/%.pdf: ./%.tikz
	@mkdir -p $(dir $@)
	@echo "Building $<"
	@pdflatex -interaction=nonstopmode -halt-on-error \
	-output-directory=$(dir $@) \
	"\def\figurefile{$<}\input{figure-build.tex}" >/dev/null
	@mv $(dir $@)/figure-build.pdf $@

cleanfig:
	rm -rf cache

clean:
	rm -rf cache build