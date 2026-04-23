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

; Inject bash into `run:` step values (default shell).
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

; Shell-specific `run:` injections. A sibling `shell:` selects the
; interpreter and overrides the default bash injection. Both literal
; shell names (`python`) and GitHub's custom-command form with the
; `{0}` script placeholder (`python {0}`) are matched.
; PowerShell, Nushell and Fish rely on their respective Zed extensions
; (wingyplus/zed-powershell, zed-extensions/nu, hasit/zed-fish); if an
; extension is not installed the injection silently falls back to plain
; text.

; Python (built-in in Zed)
(block_mapping
  (block_mapping_pair
    key: (flow_node) @_shell_key
    (#eq? @_shell_key "shell")
    value: (flow_node) @_shell_val
    (#match? @_shell_val "^['\"]?python(\\s+\\{0\\})?['\"]?$"))
  (block_mapping_pair
    key: (flow_node) @_run_key
    (#eq? @_run_key "run")
    value: [
      (flow_node
        (plain_scalar
          (string_scalar) @injection.content))
      (block_node
        (block_scalar) @injection.content)
    ]
    (#set! injection.language "python")))

; PowerShell (pwsh and powershell)
(block_mapping
  (block_mapping_pair
    key: (flow_node) @_shell_key
    (#eq? @_shell_key "shell")
    value: (flow_node) @_shell_val
    (#match? @_shell_val "^['\"]?(pwsh|powershell)(\\s+\\{0\\})?['\"]?$"))
  (block_mapping_pair
    key: (flow_node) @_run_key
    (#eq? @_run_key "run")
    value: [
      (flow_node
        (plain_scalar
          (string_scalar) @injection.content))
      (block_node
        (block_scalar) @injection.content)
    ]
    (#set! injection.language "powershell")))

; Nushell
(block_mapping
  (block_mapping_pair
    key: (flow_node) @_shell_key
    (#eq? @_shell_key "shell")
    value: (flow_node) @_shell_val
    (#match? @_shell_val "^['\"]?nu(\\s+\\{0\\})?['\"]?$"))
  (block_mapping_pair
    key: (flow_node) @_run_key
    (#eq? @_run_key "run")
    value: [
      (flow_node
        (plain_scalar
          (string_scalar) @injection.content))
      (block_node
        (block_scalar) @injection.content)
    ]
    (#set! injection.language "nu")))

; Fish
(block_mapping
  (block_mapping_pair
    key: (flow_node) @_shell_key
    (#eq? @_shell_key "shell")
    value: (flow_node) @_shell_val
    (#match? @_shell_val "^['\"]?fish(\\s+\\{0\\})?['\"]?$"))
  (block_mapping_pair
    key: (flow_node) @_run_key
    (#eq? @_run_key "run")
    value: [
      (flow_node
        (plain_scalar
          (string_scalar) @injection.content))
      (block_node
        (block_scalar) @injection.content)
    ]
    (#set! injection.language "fish")))

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
