<?php
// PHPStan's printf-param-types-format-parser.php, with the results echoed: PrintfFunctionThrowTypeExtension (unported)
// keeps PHPStan from reporting function.resultUnused on the call that throws ValueError.

namespace PrintfParamTypesFormatParser;

function doFoo(string $s, int $i): void
{
	echo sprintf('%*2$d', $i, '5');
	echo sprintf('%1$*2$d', $i, 5);
	echo sprintf('%5%', $s);
}
