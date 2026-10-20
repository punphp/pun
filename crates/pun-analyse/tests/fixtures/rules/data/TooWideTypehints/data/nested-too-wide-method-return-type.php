<?php

namespace NestedTooWideMethodReturnType;

final class Foo
{

	/**
	 * @return array<array{int, bool}>
	 */
	public function dataProvider(): array
	{
		return [
			[
				1,
				false,
			],
			[
				2,
				false,
			],
		];
	}

	/**
	 * @return array<array{int|null}>
	 */
	public function dataProvider2(): array
	{
		return [
			[
				1,
			],
			[
				2,
			],
		];
	}

}

class ParentClass
{

	/**
	 * @return array<array{int|null}>
	 */
	public function doFoo(): array
	{
		return [];
	}

}

class ChildClass extends ParentClass
{

	public function doFoo(): array
	{
		return [
			[1],
			[2],
		];
	}

}

class ChildClassNull extends ParentClass
{

	public function doFoo(): array
	{
		return [
			[null],
			[null],
		];
	}

}

class ParentClassBool
{

	/**
	 * @return array<array{bool}>
	 */
	public function doFoo(): array
	{
		return [];
	}

}

class ChildClassTrue extends ParentClassBool
{

	/**
	 * @return array<array{bool}>
	 */
	public function doFoo(): array
	{
		return [
			[true],
		];
	}

}

final class WebhookTest
{

	/**
	 * @return array<string, int|string|bool>
	 */
	public function dataTest(int $remoteId, string $hookUrl): array
	{
		return [
			'id' => $remoteId,
			'url' => $hookUrl,
			'push_events' => true,
		];
	}

}
