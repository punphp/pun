<?php

namespace Pr4372NullCoalesce;

function (array $matches): void {
	$stats = [];
	for ($i = 0; $i < count($matches); $i++) {
		$author = $matches[$i][1];
		$files = (int) $matches[$i][3];
		$insertions = (int) ($matches[$i][4] ?? 0);

		$stats[$author]['commits'] = ($stats[$author]['commits'] ?? 0) + 1;
		$stats[$author]['files'] = ($stats[$author]['files'] ?? 0) + $files;
		$stats[$author]['insertions'] = ($stats[$author]['insertions'] ?? 0) + $insertions;
	}
};
