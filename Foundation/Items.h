#include "Item.h"
#include "Weapon.h"
#include "GameObject.h"
#include "Enum.h"


namespace Foundation
{

	class Items
	{
	public:
		//Static call to produce an item object. Can be anything from barrel to wonderous weapon.
		static Item* Items::GenerateItemRandomItem(enum eQuality = Exceptional, int level = 0);
		static Weapon* Items::GenerateRandomWeapon(enum eQuality = Exceptional, int level = 0);
		static void Items::ItemReport();
		static void Items::ResetReport();
	protected:
		static void Items::ObjectCount(Item* i);
	private:

	};

}
