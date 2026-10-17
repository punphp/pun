<?php

namespace PropertyExists;

class Model
{

}

class Defaults
{
	public function defaults(Model $model): void
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
			if (property_exists($model, $column)) {
				echo $model->{$column};
			}
		}
	}
}
