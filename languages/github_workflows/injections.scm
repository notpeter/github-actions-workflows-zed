; Inject the ghactions expression grammar into YAML string scalars
; that contain a `${{ ... }}` expression.
([
  (string_scalar)
  (block_scalar)
  (double_quote_scalar)
  (single_quote_scalar)
] @injection.content
  (#match? @injection.content "[$][{][{]")
  (#set! injection.language "GitHub Actions Expression"))

; Inject bash into `run:` step values.
(block_mapping_pair
  key: (flow_node (plain_scalar (string_scalar) @_key))
  value: (block_node (block_scalar) @injection.content)
  (#eq? @_key "run")
  (#set! injection.language "bash"))

(block_mapping_pair
  key: (flow_node (plain_scalar (string_scalar) @_key))
  value: (flow_node (plain_scalar (string_scalar) @injection.content))
  (#eq? @_key "run")
  (#set! injection.language "bash"))
