RARAS	= ./raras
RARLD	= ./rarld

%.ri: %.rs
	cpp -Istdlib < $< > $@

%.ro: %.ri
	$(RARAS) -o $@ $<

%.rar: %.ro
	$(RARLD) $< > $@

all: keccak.rar 
