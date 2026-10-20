<?php // lint >= 8.1

namespace Bug12951;

function (): void {
	\Bug12951Core\NumberFormatter::doBar();
	\Bug12951Polyfill\NumberFormatter::doBar();

};
