<?php
namespace TraitInitializers;
const SCANNED_ZERO = null ?? 0;
class Scanned { public const FALLBACK = false ? 128 : 256; }
trait ScannedInputs {
    public const FLAGS = self::ZERO ?: parent::FLAGS;
    public const FORWARD = self::LATER ?: Scanned::FALLBACK;
    public const LATER = SCANNED_ZERO;
}
function unanalysedBody(): void { missingInScannedFile(); }
