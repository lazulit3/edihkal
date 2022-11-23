;; Make rust-analyzer use edihkal_tracing with this feature flag enabled even though it's disabled by default.
(lsp-mode . (lsp-rust-features '("test-helpers")))
