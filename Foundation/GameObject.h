#pragma once
#include "Item.h"

namespace Foundation
{
	class GameObject :
		public Item
	{
	public:
		GameObject(enum eQuality quality, int level);
		~GameObject();
		void Draw(void);
	protected:
	private:
		short SetItemFumbleRate(void);
	};

}

