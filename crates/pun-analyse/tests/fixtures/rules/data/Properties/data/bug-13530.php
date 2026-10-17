<?php // lint >= 8.0

namespace Bug13530;

/** @property-read string $url */
class HelloWorld
{

	private string $url;

	public function __construct(string $url) {
		$this->url = $url;
	}

	public function __get(string $key): mixed {
		if ($key === 'url') {
			return $this->url;
		}

		return NULL;
	}

}
