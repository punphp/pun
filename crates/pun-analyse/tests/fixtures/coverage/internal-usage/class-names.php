<?php

namespace VendorNames {

	/** @internal */
	class Secret
	{
		public static int $count = 0;

		/** @internal */
		public static int $hidden = 0;

		public const PUBLIC = 1;

		public string $name = '';

		public static function make(): void
		{
		}

		public function __toString(): string
		{
			return '';
		}
	}

	/** @internal */
	interface SecretInterface
	{
	}

	/** @internal */
	trait SecretTrait
	{
	}

	/** @internal */
	enum SecretEnum
	{
		case A;
	}

	/** @internal */
	#[\Attribute]
	class SecretAttribute
	{
	}

	/** @internal */
	class SecretException extends \Exception
	{
	}

	class Open
	{
		/** @internal */
		public function __toString(): string
		{
			return '';
		}

		/** @internal */
		public string $hiddenProperty = '';
	}

	class Inside
	{
		public function run(Secret $s): Secret
		{
			Secret::make();
			echo Secret::$count;
			return new Secret();
		}
	}

}

namespace AppNames {

	use VendorNames\Open;
	use VendorNames\Secret;
	use VendorNames\SecretAttribute;
	use VendorNames\SecretEnum;
	use VendorNames\SecretException;
	use VendorNames\SecretInterface;
	use VendorNames\SecretTrait;

	class Child extends Secret implements SecretInterface
	{
		use SecretTrait;

		public Secret $property;

		#[SecretAttribute]
		public function run(Secret $s, ?SecretInterface $i = null): Secret|SecretEnum
		{
			if ($s instanceof Secret) {
			}
			try {
			} catch (SecretException $e) {
			}
			Secret::make();
			echo Secret::$count;
			echo Secret::$hidden;
			echo Secret::PUBLIC;
			echo (string) $s;
			echo $s->name;
			$open = new Open();
			echo (string) $open;
			echo $open->hiddenProperty;
			$f = function (Secret $x): Secret {
				return $x;
			};
			$g = fn (Secret $x): Secret => $x;
			$anonymous = new class extends Secret implements SecretInterface {
				public function anonymous(Secret $x): Secret
				{
					return $x;
				}
			};
			return new Secret();
		}
	}

	interface ChildInterface extends SecretInterface
	{
	}

	enum ChildEnum implements SecretInterface
	{
		case B;
	}

	function helper(Secret $s): Secret
	{
		return $s;
	}

}
