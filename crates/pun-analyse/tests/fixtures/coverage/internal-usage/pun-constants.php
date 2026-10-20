<?php

namespace PunInternalConstants {

	class Base
	{
		/** @internal */
		public const HIDDEN = 1;
	}

	/** @internal */
	trait InternalTrait
	{
		public const IN_TRAIT = 1;
	}

}

namespace PunOther {

	use PunInternalConstants\Base;

	class Child extends Base
	{
		public function read(Base $base): void
		{
			echo parent::HIDDEN;
			echo self::HIDDEN;
			echo static::HIDDEN;
			echo $this::HIDDEN;
			echo $base::HIDDEN;
			echo Missing::HIDDEN;
			echo \PunInternalConstants\InternalTrait::IN_TRAIT;
		}
	}

}
