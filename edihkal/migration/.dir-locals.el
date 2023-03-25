;; Make rust-analyzer use migration with cli feature flag enabled even though it's disabled by default.
(lsp-mode . (lsp-rust-features '("cli")))
