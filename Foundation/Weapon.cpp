#include "Weapon.h"

namespace Foundation
{
	short Weapon::SetItemFumbleRate(void)
	{
		short i = 0;

		return i;
	}

	void Weapon::Draw(void){}

	Weapon::Weapon(enum eQuality quality, int level) : Item(quality, level)
	{
		m_sFumbleRate = SetItemFumbleRate();
	}


	Weapon::~Weapon()
	{
	}
}
