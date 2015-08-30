#pragma once
#include "Item.h"

namespace Foundation
{
	class Weapon :
		public Item
	{
	public:
		//Weapon is a result of the base class Item. May need to think out some weapon typing and such later.
		//Animations based on weapons types would be a nice addition later.
		Weapon(enum eQuality quality, int level);
		~Weapon();
		void Draw(void);
	protected:
	private:
		short SetItemFumbleRate(void);
		

	};
 }