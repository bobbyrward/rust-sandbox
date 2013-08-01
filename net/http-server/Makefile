RUSTC?=rustc
RUSTCFLAGS?=
RUSTLDFLAGS?=


http_server: libhttp_parser.a src/main.rs
	$(RUSTC) $(RUSTCFLAGS) $(RUSTLDFLAGS) $< -o $@


libhttp_parser.a: src/http_server/http/http_parser.o
	$(AR) rcs $@ $<


src/http_server/http/ttp_parser.o: src/http_server/http//http_parser.c
	$(CC) $(CFLAGS) $< -o $@ -c


.PHONY: clean


clean:
	find ./ -name '*~' -or -name '*.dSYM' -print0 | xargs -0 rm -r
