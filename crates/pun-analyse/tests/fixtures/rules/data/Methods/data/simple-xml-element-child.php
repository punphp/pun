<?php

namespace SimpleXmlElementChild;

class CustomXMLElement extends \SimpleXMLElement
{
	#[\Override]
	public function addChild(string $qualifiedName, ?string $value = null, ?string $namespace = null): ?static
	{
		return parent::addChild($qualifiedName, $this->escapeInput($value), $namespace);
	}

	private function escapeInput(?string $value): ?string
	{
		if ($value === null) {
			return null;
		}
		return htmlspecialchars((string) normalizer_normalize($value), ENT_XML1, 'UTF-8');
	}
}
