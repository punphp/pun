<?php

namespace Bug12951;

function (): void {
	echo \Bug12951Core\NumberFormatter::$prop;
	echo \Bug12951Polyfill\NumberFormatter::$prop;
};
