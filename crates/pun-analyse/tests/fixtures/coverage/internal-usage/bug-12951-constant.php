<?php

namespace Bug12951;

function (): void {
	echo \Bug12951Core\NumberFormatter::NUMERIC_COLLATION;
	echo \Bug12951Polyfill\NumberFormatter::NUMERIC_COLLATION;
};
