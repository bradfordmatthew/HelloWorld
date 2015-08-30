#include<cstdlib>
#include<ctime>

using namespace std;


#include "Items.h"

namespace Foundation
{
	int m_itemCount;
	int m_iLegendaryCount;
	int m_iGodCount;
	Item* Items::GenerateItemRandomItem(eQuality quality, int level)
	{
		Item* i = new GameObject(quality, level);
		ObjectCount(i);
		return i;
	}

	Weapon* Items::GenerateRandomWeapon(eQuality quality, int level)
	{
		Weapon* w = new Weapon(quality, level);
		ObjectCount(w);
		return w;
	}

	void Items::ItemReport()
	{
		std::cout << "\r\nItems Generated: " << m_itemCount << ".\r\n";
		std::cout << "Legendary Items Generated: " << m_iLegendaryCount << ".\r\n";
		std::cout << "God Items Created: " << m_iGodCount << ".\r\n";
	}

	void Items::ResetReport()
	{
		m_itemCount = 0;
		m_iLegendaryCount = 0;
		m_iGodCount = 0;
	}

	void Items::ObjectCount(Item* i)
	{
		m_itemCount++;
		if ((*i).m_bGod)
			m_iGodCount++;
		if ((*i).m_bLegendary)
			m_iLegendaryCount++;
	}
}
