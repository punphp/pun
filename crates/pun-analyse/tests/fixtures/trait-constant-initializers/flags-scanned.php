<?php
namespace TraitFlagInputs;
const FORWARD_GLOBAL = null ?? 0;
trait ScannedFlags {
    public const FLAGS = self::NEXT ?: parent::TARGET;
    public const NEXT = FORWARD_GLOBAL;
}
function unanalysed(): void { missingFlagBody(); }
