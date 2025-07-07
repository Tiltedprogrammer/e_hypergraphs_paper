LICS24="E-Hypergraphs-LICS24"
LICS24_PATH="LICS"
LICS24_SOURCE=$(LICS24).tex

LICS_CLOSED="E-graphs-with-bindings"
LICS_CLOSED_PATH="LICS_2025/e_graphs_with_bindings"
LICS_CLOSED_SOURCE=$(LICS_CLOSED).tex


LICS25_MONOIDAL_EGRAPHS="Monoidal_EGraphs"
LICS25_MONOIDAL_EGRAPHS_PATH="LICS_2025/monoidal_e_graphs/sections"
LICS25_MONOIDAL_EGRAPHS_SOURCE=$(LICS25_MONOIDAL_EGRAPHS).tex

CLOSED="closed_monoidal"
CLOSED_PATH="closed_monoidal"
CLOSED_SOURCE=$(CLOSED).tex

ARXIV="E-Hypergraphs-arxiv"
ARXIV_PATH="arxiv"
ARXIV_SOURCE=$(ARXIV).tex

SLIDES="slides"
SLIDES_PATH="slides"
SLIDES_SOURCE=$(SLIDES).tex


SLOTTED="SlottedNotes"
SLOTTED_PATH="POPL2026/notes"
SLOTTED_SOURCE=$(SLOTTED).tex


PDFLATEX= pdflatex -file-line-error -shell-escape -interaction=nonstopmode
XELATEX= xelatex -file-line-error -shell-escape -interaction=nonstopmode -pdf

CLEANALL= rm -rf auto _* *.aux *.log *.nav *.out *.snm *.toc *~ *.blg *.bbl *.cut *.dvi *.xcp *.fdb_latexmk *.fls *.synctex.gz *.pdf

.PHONY: arxiv closed lics slides

arxiv:
	cd $(ARXIV_PATH) && $(CLEANALL) && $(PDFLATEX) $(ARXIV_SOURCE) && bibtex $(ARXIV) && $(PDFLATEX) $(ARXIV_SOURCE) && $(PDFLATEX) $(ARXIV_SOURCE)

closed:
	cd $(CLOSED_PATH) && $(CLEANALL) && $(PDFLATEX) $(CLOSED_SOURCE) && bibtex $(CLOSED) && $(PDFLATEX) $(CLOSED_SOURCE) && $(PDFLATEX) $(CLOSED_SOURCE)

lics:
	cd $(LICS24_PATH) && $(CLEANALL) && $(PDFLATEX) $(LICS24_SOURCE) && bibtex $(LICS24) && $(PDFLATEX) $(LICS24_SOURCE) && $(PDFLATEX) $(LICS24_SOURCE)

lics_closed:
	cd $(LICS_CLOSED_PATH) && $(CLEANALL) && $(PDFLATEX) $(LICS_CLOSED_SOURCE) && bibtex $(LICS_CLOSED) && $(PDFLATEX) $(LICS_CLOSED_SOURCE) && $(PDFLATEX) $(LICS_CLOSED_SOURCE)

lics_monoidal_egraph:
	cd $(LICS25_MONOIDAL_EGRAPHS_PATH) && $(CLEANALL) && $(PDFLATEX) $(LICS25_MONOIDAL_EGRAPHS_SOURCE) && bibtex $(LICS25_MONOIDAL_EGRAPHS) && $(PDFLATEX) $(LICS25_MONOIDAL_EGRAPHS_SOURCE) && $(PDFLATEX) $(LICS25_MONOIDAL_EGRAPHS_SOURCE)	

slides:
	cd $(SLIDES_PATH) && $(CLEANALL) && $(XELATEX) $(SLIDES_SOURCE)

slotted:
	cd $(SLOTTED_PATH) && $(CLEANALL) && $(PDFLATEX) $(SLOTTED_SOURCE) && bibtex $(SLOTTED) && $(PDFLATEX) $(SLOTTED_SOURCE) && $(PDFLATEX) $(SLOTTED_SOURCE)

clean:
	echo "TODO"

cleanall: clean
	echo "TODO"
