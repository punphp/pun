<?php

namespace Bug8668;

class Sample {
	private static string $sample = 'abc';

	public function test(): void {
		echo $this->sample;
		echo isset($this->sample);

		echo self::$sample; // ok
	}
}

class Sample2 {
	private static $sample = 'abc';

	public function test(): void {
		echo $this->sample;
		echo isset($this->sample);

		echo self::$sample; // ok
	}
}
