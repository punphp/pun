<?php

namespace Bug13123;

class ControlGroup {}

class Container {
	protected ?ControlGroup $currentGroup = null;
}

class Multiplier extends Container {
	protected function createContainer(): Container
	{
		$control = new Container();
		$control->currentGroup = $this->currentGroup;

		return $control;
	}

	protected function createContainer2(Container $control): Container
	{
		$control->currentGroup = $this->currentGroup;

		return $control;
	}


}
