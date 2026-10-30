<?php
namespace InitializerScopes;
class MetadataOnly {
    const ZERO = 0;
    const SPLIT = self::ZERO ?: (true ? 1 : 4);
    function unrelated(): void { unrelated_missing_function(); }
}
