<?php

final class TabIndent
{
	/**
	 * @var list<int>
	 */
	private array $ids = [];

	public function getIds(): array
	{
		return $this->ids;
	}
}
-----
<?php

final class TabIndent
{
	/**
	 * @var list<int>
	 */
	private array $ids = [];

	/**
     * @return list<int>
     */
    public function getIds(): array
	{
		return $this->ids;
	}
}
