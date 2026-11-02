<?php

namespace Bug6233;

final class TestClass {
	/**
	 * @throws \Exception
	 **/
	public function __invoke() {
		throw new \Exception();
	}
}

final class TestClass2 {
	/**
	 * @throws \Exception
	 **/
	public function __invoke() {
		throw new \Exception();
	}
}

final class Container {
	/**
	 * @throws \Exception
	 **/
	public function test(TestClass $class) {
		$class();
	}
	/**
	 * @throws \Exception
	 **/
	public function testNew() {
		(new TestClass)();
	}
	/**
	 * @param TestClass|TestClass2 $class
	 *
	 * @throws \Exception
	 **/
	public function testUnion(object $class) {
		$class();
	}
}
