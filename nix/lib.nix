{ lib, ... }:
let
    _filterAttrsRec = filter: attrs: prefix:
        let
            filtered = lib.filterAttrs (k: v: (filter (prefix + k) v)) attrs;
            cleaned = lib.mapAttrs (k: v: if builtins.isAttrs v then _filterAttrsRec filter v (prefix + k) else v ) filtered;
        in cleaned;
in lib // {
    filterAttrsRec = filter: attrs: (_filterAttrsRec filter attrs "");
}