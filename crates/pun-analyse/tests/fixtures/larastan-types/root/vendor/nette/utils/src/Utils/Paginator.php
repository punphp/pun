<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;


/**
 * Paginating math.
 *
 * @property   int $page
 * @property-read int $firstPage
 * @property-read ?int $lastPage
 * @property-read int<0,max> $firstItemOnPage
 * @property-read int<0,max> $lastItemOnPage
 * @property   int $base
 * @property-read bool $first
 * @property-read bool $last
 * @property-read ?int<0,max> $pageCount
 * @property   positive-int $itemsPerPage
 * @property   ?int<0,max> $itemCount
 * @property-read int<0,max> $offset
 * @property-read ?int<0,max> $countdownOffset
 * @property-read int<0,max> $length
 */
class Paginator
{
	use Nette\SmartObject;

	private int $base = 1;

	/** @var positive-int */
	private int $itemsPerPage = 1;

	private int $page = 1;

	/** @var ?int<0, max> */
	private ?int $itemCount = null;


	public function setPage(int $page): static
	{}


	public function getPage(): int
	{}


	public function getFirstPage(): int
	{}


	public function getLastPage(): ?int
	{}


	/**
	 * Returns the 1-based index of the first item on the current page, or 0 if the page is empty.
	 * @return int<0, max>
	 */
	public function getFirstItemOnPage(): int
	{}


	/**
	 * Returns the 1-based index of the last item on the current page.
	 * @return int<0, max>
	 */
	public function getLastItemOnPage(): int
	{}


	public function setBase(int $base): static
	{}


	public function getBase(): int
	{}


	/**
	 * Returns zero-based page number.
	 * @return int<0, max>
	 */
	protected function getPageIndex(): int
	{}


	public function isFirst(): bool
	{}


	public function isLast(): bool
	{}


	/**
	 * @return ?int<0, max>
	 */
	public function getPageCount(): ?int
	{}


	public function setItemsPerPage(int $itemsPerPage): static
	{}


	/**
	 * @return positive-int
	 */
	public function getItemsPerPage(): int
	{}


	public function setItemCount(?int $itemCount = null): static
	{}


	/**
	 * @return ?int<0, max>
	 */
	public function getItemCount(): ?int
	{}


	/**
	 * Returns the absolute index of the first item on current page.
	 * @return int<0, max>
	 */
	public function getOffset(): int
	{}


	/**
	 * Returns the absolute index of the first item on current page in countdown paging.
	 * @return ?int<0, max>
	 */
	public function getCountdownOffset(): ?int
	{}


	/**
	 * Returns the number of items on current page.
	 * @return int<0, max>
	 */
	public function getLength(): int
	{}
}
