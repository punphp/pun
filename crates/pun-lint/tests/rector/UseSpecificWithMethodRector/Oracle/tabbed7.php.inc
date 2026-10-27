<?php

namespace App\Tests\Tabbed7 {
	use PHPUnit\Framework\MockObject\MockObject;
	use PHPUnit\Framework\TestCase;
	use Symfony\Component\Form\Test\TypeTestCase;

	final class TabbedFormTest extends TypeTestCase
	{
		private MockObject $someMock;
	}

	/**
	 * @covers \Foo
	 */
	final class TabbedCoversTest extends TestCase
	{
		/**
		 * @return Event|MockObject
		 */
		private function createEvent(): MockObject
		{
			return $this->createMock(Event::class);
		}
	}
}
