#include "Items.h"
#include "Item.h"

void CleanUp(vector<Foundation::Item*> items)
{
	for (vector<Foundation::Item*>::iterator i = items.begin(); i != items.end(); ++i)
	{
		(*i)->~Item();
		*i = 0;
	}
}

int _tmain(int argc, _TCHAR* argv[])
{
	vector<Foundation::Item*> container;
	vector<Foundation::Weapon*> weapons;
	Foundation::Items r;

	char s = 'y';
	while (s != 'n' && s != 'N')
	{
		CleanUp(container);
		container.clear();
		r.ResetReport();

		for (int x = 0; x < 100000; x++)
		{
			container.push_back(r.GenerateItemRandomItem());
			weapons.push_back(r.GenerateRandomWeapon());
		}

		for (vector<Foundation::Item*>::iterator itr = container.begin(); itr != container.end(); itr++)
		{
			(*itr)->Print();
		}

		for each (Foundation::Weapon* weapon in weapons)
		{
			weapon->Print();
		}

		r.ItemReport();

		std::cout << "Continue? Y continue, N to exit: ";
		std::cin >> s;
	}


	return 0;
}
