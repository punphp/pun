<?php

namespace FunctionInternalTagOne {

	/** @internal */
	function doInternal()
	{

	}

	function doNotInternal()
	{

	}

	function (): void {
		doInternal();
		doNotInternal();
	};

}

namespace FunctionInternalTagOne\Test {

	function (): void {
		\FunctionInternalTagOne\doInternal();
		\FunctionInternalTagOne\doNotInternal();
	};

}

namespace FunctionInternalTagTwo {

	function (): void {
		\FunctionInternalTagOne\doInternal();
		\FunctionInternalTagOne\doNotInternal();
	};

}

namespace {

	function (): void {
		\FunctionInternalTagOne\doInternal();
		\FunctionInternalTagOne\doNotInternal();
	};

	/** @internal */
	function doInternalWithoutNamespace()
	{

	}

	function doNotInternalWithoutNamespace()
	{

	}

	function (): void {
		doInternalWithoutNamespace();
		doNotInternalWithoutNamespace();
	};

}

namespace SomeNamespace {

	function (): void {
		\doInternalWithoutNamespace();
		\doNotInternalWithoutNamespace();
	};

}
