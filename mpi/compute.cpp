#include <stdio.h>
#include <mpi.h>
#include <boost/multiprecision/cpp_dec_float.hpp>

#define FLOAT_PRECISION 1000000
#define let auto

typedef boost::multiprecision::cpp_dec_float<FLOAT_PRECISION> boost_float;

int main(int argc, char ** argv) {
    int ierr;

    ierr = MPI_Init(&argc, &argv);
    std::cout << "Program is starting" << std::endl;

    int my_id, num_procs;
    ierr = MPI_Comm_rank(MPI_COMM_WORLD, &my_id);
    ierr = MPI_Comm_size(MPI_COMM_WORLD, &num_procs);

    MPI_Barrier(MPI_COMM_WORLD);

    std::cout << "Process " << my_id << " checking in out of " << num_procs << "procs" << std::endl;


    ierr = MPI_Finalize();
}

boost_float calculate(size_t start, size_t end) {
    if(start > end) {
        return boost_float::zero();
    } else {
        boost_float sum = boost_float::zero();
        for(size_t i = start; i <= end; i++) {
            boost_float sign = i % 2 == 0 ? 1 : -1;
            let numerator = sign;
            boost_float denominator_1 = 1 + (2 * i);
            boost_float denominator_2 = boost_float(3).pow2(i);
            //let denominator = denominator_1 * denominator_2;

            let term = numerator / denominator_1;
            sum = sum + term;
        }

        return sum;
    }
}
