<?php

namespace ThrowsCoverage;

class CheckedException extends \Exception {}

class UncheckedException extends \RuntimeException {}

class SubChecked extends CheckedException {}

/** @throws CheckedException */
function throwsChecked(): void
{
	throw new CheckedException();
}

/** @throws UncheckedException */
function throwsUnchecked(): void
{
	throw new UncheckedException();
}

function noTag(): void
{
}

function callsChecked(): void
{
	throwsChecked();
	throwsUnchecked();
	throw new SubChecked();
}

/** @throws CheckedException */
function coversSubclass(): void
{
	throw new SubChecked();
}

/** @throws CheckedException|UncheckedException */
function tooWide(): void
{
	throwsUnchecked();
}

/** @throws CheckedException */
function implicitOnly(): void
{
	noTag();
}

function deadCatches(): void
{
	try {
		throwsChecked();
	} catch (SubChecked $e) {
		// a subclass of the thrown type: not dead
	} catch (UncheckedException $e) {
		// dead
	}

	try {
		noTag();
	} catch (CheckedException $e) {
		// implicit throw point: dead without implicitThrows only
	}

	try {
		throwsUnchecked();
	} catch (\LogicException $e) {
		// dead, an unchecked-class dead catch is still reported unless reportUncheckedExceptionDeadCatch is off
	}

	try {
		$x = 1;
	} catch (UncheckedException $e) {
		// dead
	} catch (\Error $e) {
		// dead
	}
}

interface HasThrows
{
	/** @throws CheckedException */
	public function run(): void;

	/** @throws void */
	public function quiet(): void;

	public function untagged(): void;
}

final class Implementation implements HasThrows
{
	/** @throws \Exception */
	public function run(): void
	{
		throw new \Exception();
	}

	/** @throws UncheckedException */
	public function quiet(): void
	{
		throw new UncheckedException();
	}

	/** @throws UncheckedException */
	public function untagged(): void
	{
		throw new UncheckedException();
	}

	/** @throws CheckedException */
	private function privateTooWide(): void
	{
	}

	public function caller(): void
	{
		$this->privateTooWide();
		$this->run();
	}
}

class Hooks
{
	public int $value {
		/** @throws CheckedException */
		get {
			throw new SubChecked();
		}
		set {
			throwsChecked();
		}
	}

	private int $quiet {
		/** @throws UncheckedException */
		get => 1;
	}
}
