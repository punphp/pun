<?php declare(strict_types=1);

namespace Bug10819;

use DateTime;
use ValueError;

class HelloWorld
{
	public function checkDate(string $dateString): bool
	{
		try {
			DateTime::createFromFormat('Y-m-d', $dateString);

			return true;
		} catch (ValueError) {
			return false;
		}
	}
}
