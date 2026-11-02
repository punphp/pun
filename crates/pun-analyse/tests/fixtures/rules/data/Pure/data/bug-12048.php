<?php

namespace Bug12048;

class HelloWorld
{
	/** @phpstan-pure */
	public function sayHello(string $s): string
	{
		$a = md5( $s );
		$a .= hash( 'md5', $s );
		$a .= hash_hmac( 'sha1', $s, 'b' );

		$a .= hash( 'sha256', $s );
		$a .= hash_hmac( 'sha256', $s, 'b' );

		return $a;
	}
}
