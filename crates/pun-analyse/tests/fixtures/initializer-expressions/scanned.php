<?php
namespace InitializerExpressionFallback;
const SCANNED_CALL = number();
class Scanned { public const CALL = number(); public const ARRAY_VALUE = ['value' => number()]; }
function unanalysed(): void { absentInUnanalysedBody(); }
