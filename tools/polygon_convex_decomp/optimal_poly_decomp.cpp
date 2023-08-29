// The following tool uses CGAL, the Computational Geometry Algorithms Library:
// https://www.cgal.org
// Specifically, this tool uses a package, 2D Polygon Partitioning, created by Susan Hert:
// https://doc.cgal.org/latest/Partition_2/index.html#Chapter_2D_Polygon_Partitioning
// The (modified) code below comes from documentation for the optimal_convex_partition_2() function:
// https://doc.cgal.org/latest/Partition_2/group__PkgPartition2Ref.html#ga3ca9fb1f363f9f792bfbbeca65ad5cc5
// License: GPL
// https://www.gnu.org/licenses/gpl-3.0.html#license-text

// to generate CMakeLists.txt:
// /opt/homebrew/Cellar/cgal/5.6/bin/cgal_create_CMakeLists

#include <CGAL/Exact_predicates_inexact_constructions_kernel.h>
#include <CGAL/Partition_traits_2.h>
#include <CGAL/partition_2.h>
#include <cassert>
#include <list>

typedef CGAL::Exact_predicates_inexact_constructions_kernel K;
typedef CGAL::Partition_traits_2<K>                         Traits;
typedef Traits::Polygon_2                                   Polygon_2;
typedef Traits::Point_2                                     Point_2;
typedef std::list<Polygon_2>                                Polygon_list;

void make_polygon(Polygon_2& polygon)
{
   polygon.push_back(Point_2(12.24, -16.85));
   polygon.push_back(Point_2(47.55, -15.45));
   polygon.push_back(Point_2(19.81, 6.44));
   polygon.push_back(Point_2(29.39, 40.45));
   polygon.push_back(Point_2(-0.00, 20.83));
   polygon.push_back(Point_2(-29.39, 40.45));
   polygon.push_back(Point_2(-19.81, 6.44));
   polygon.push_back(Point_2(-47.55, -15.45));
   polygon.push_back(Point_2(-12.24, -16.85));
   polygon.push_back(Point_2(0.00, -50.00));
}

int main()
{
   Polygon_2             polygon;
   Polygon_list          partition_polys;

   make_polygon(polygon);
   CGAL::optimal_convex_partition_2(polygon.vertices_begin(),
                                    polygon.vertices_end(),
                                    std::back_inserter(partition_polys));

   assert(CGAL::partition_is_valid_2(polygon.vertices_begin(),
                                     polygon.vertices_end(),
                                     partition_polys.begin(),
                                     partition_polys.end()));

   for (const Polygon_2& poly : partition_polys) {
     for(Point_2 p : poly.container()) {
        std::cout << "(" << p << "), ";
      }
     std::cout << std::endl;
   }

   return 0;
}