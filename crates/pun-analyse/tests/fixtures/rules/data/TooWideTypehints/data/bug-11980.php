<?php

namespace Bug11980;

final class Demo
{

	/**
	 * @param array<int, array<string, int>> $tokens
	 * @param int                            $stackPtr
	 *
	 * @return int|void
	 */
	public function process($tokens, $stackPtr)
	{
		if (empty($tokens[$stackPtr]['nested_parenthesis']) === false) {
			// Not a stand-alone statement.
			return;
		}

		$end = 10;

		if ($tokens[$end]['code'] !== 10
			&& $tokens[$end]['code'] !== 20
		) {
			// Not a stand-alone statement.
			return $end;
		}
	}

	/**
	 * @param array<int, array<string, int>> $tokens
	 * @param int                            $stackPtr
	 *
	 * @return int|void
	 */
	public function process2($tokens, $stackPtr)
	{
		if (empty($tokens[$stackPtr]['nested_parenthesis']) === false) {
			// Not a stand-alone statement.
			return null;
		}

		$end = 10;

		if ($tokens[$end]['code'] !== 10
			&& $tokens[$end]['code'] !== 20
		) {
			// Not a stand-alone statement.
			return $end;
		}

		return 1;
	}

	/** @return int|void */
	public function process3( int $code ) {

		if ( $code === \T_CLASS ) {
			return $this->process_class( $code );
		}

		$this->process_function( $code );
	}

	/** @return int */
	public function process_class(int $code) {
		return $code;
	}

	/** @return void */
	public function process_function(int $code) {
	}
}
