#!%NU_PATH%
def --wrapped main [...original_args] {
  let data = '%CONTEXT%' | decode base64 | decode | from json
  let target_path = $data | get target
  let envelop = $data | get env

  if $envelop.cwd != null { cd $envelop.cwd }
  let $envvars = $envelop.variables | items { |k, v|
    let v = $envelop.variables | get $k
    let value = $v.value | default {
      mut original = $env | get -o $k
      if $k == 'PATH' and $original != null {
        $original = $original | str join (char esep)
      }
      $original | default -e null
    }
    [$k, ([$v.prefix $value $v.suffix] | where $it != null | str join $v.separator)]
  } | into record

  with-env $envvars {
    run-external $target_path ...$envelop.args ...$original_args
  }
}
