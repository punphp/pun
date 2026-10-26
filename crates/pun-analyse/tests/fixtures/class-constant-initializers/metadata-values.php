<?php
namespace InitializerMetadataRoots;

class Values {
    const SPLIT_FILTER = self::ZERO ?: (true ? 1 : 4);
    const SPLIT_COALESCE = self::ZERO ?? 1;
    const ZERO = 0;
    final const FINAL_SUM = (1.5 + 0.5) * 2;
    final const FINAL_SPLIT = self::ZERO ?: (true ? 1 : 4);
    function unrelated(): void { unrelated_missing_function(); }
}
