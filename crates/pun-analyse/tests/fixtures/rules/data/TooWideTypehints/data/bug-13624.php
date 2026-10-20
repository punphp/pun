<?php

namespace Bug13624;

class Example
{

	/** @var ?string */
	private $exampleProperty;
	/** @var string */
	private $exampleProperty2;
	/** @var ?int */
	private $exampleProperty3;
	/** @var int */
	private $exampleProperty4;
	/** This is fine if uninitialized property warnings are off */
	private ?int $exampleProperty5;


	/** @param array<string, int|float|string|null> $row */
	public static function from_db_row(array $row): self
	{
		$course = new self();
		foreach ($row as $key => $value) {
			$course->{$key} = $value;
		}
		return $course;
	}

}
