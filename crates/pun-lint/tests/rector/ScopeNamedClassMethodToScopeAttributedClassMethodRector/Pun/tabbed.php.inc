<?php

namespace App {
	use Illuminate\Database\Eloquent\Model;

	class A extends Model
	{
		public function scopeActive($q) {}
	}
}
