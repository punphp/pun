<?php

namespace Bug12793;

class Service
{
	protected function defaults(Model $model): void
	{
		$columns = [
			'getCreatedByColumn',
			'getUpdatedByColumn',
			'getDeletedByColumn',
			'getCreatedAtColumn',
			'getUpdatedAtColumn',
			'getDeletedAtColumn',
		];

		foreach ($columns as $column) {
			if (method_exists($model, $column)) {
				$model->{$column}();
			}
		}
	}
}

class Model {}
