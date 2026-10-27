<?php

namespace App {
	use Illuminate\Database\Eloquent\Model;

	class A extends Model
	{
		protected $fillable = [
			'a',
		];

		public function scopeActive($q) {}
	}
}
