<?php declare(strict_types=1);

namespace Bug4532;

use AllowDynamicProperties;

/**
 * @property int|null $status
 */
#[AllowDynamicProperties]
class Entity
{
	public const STATUS_OFF = 0;
	public const STATUS_ON = 1;

	/** @var array<int, string> */
	public static array $statuses = [
		self::STATUS_OFF => 'Off',
		self::STATUS_ON => 'On',
	];
}

function (): void {
	$entity = new Entity;
	echo Entity::$statuses[$entity->status];
};
