<?php // lint >= 8.1

namespace Bug12951;

function (): void {
	new \Bug12951Core\NumberFormatter();
	new \Bug12951Polyfill\NumberFormatter();
};
