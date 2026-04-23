; Inject the ghactions expression grammar into YAML string scalars
; that contain a `${{ ... }}` expression.
([
  (string_scalar)
  (block_scalar)
  (double_quote_scalar)
  (single_quote_scalar)
] @injection.content
  (#match? @injection.content "[$][{][{]")
  (#set! injection.language "ghactions"))

; Inject bash into `run:` step values.
(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @_key))
  value: (block_node
    (block_scalar) @injection.content)
  (#eq? @_key "run")
  (#set! injection.language "bash"))

(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @_key))
  value: (flow_node
    (plain_scalar
      (string_scalar) @injection.content))
  (#eq? @_key "run")
  (#set! injection.language "bash"))

; GitHub actions: JavaScript for workflow scripting (inline and block)
(block_mapping
  (block_mapping_pair
    key: (flow_node) @_uses
    (#eq? @_uses "uses")
    value: (flow_node) @_actions_ghs
    (#match? @_actions_ghs "^actions/github-script"))
  (block_mapping_pair
    key: (flow_node) @_with
    (#eq? @_with "with")
    value: (block_node
      (block_mapping
        (block_mapping_pair
          key: (flow_node) @_run
          (#eq? @_run "script")
          value: [
            (flow_node
              (plain_scalar
                (string_scalar) @injection.content))
            (block_node
              (block_scalar) @injection.content)
          ]
          (#set! injection.language "javascript"))))))
