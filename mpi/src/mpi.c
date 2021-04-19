#include <mpi.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define REQ_PRECISION 10000
#define REQ_RANGE 1000000

extern void *rs_make_vec();
extern void *rs_calc_to_cstr(uint32_t, size_t, size_t);
extern void rs_add_result(void *, char *);
extern void rs_sum_results(void *, uint32_t);

struct calc_req {
  size_t start;
  size_t end;
  uint32_t precision;
};

void collector(int num_procs) {
  printf("Collector checking in with %d generators\n", num_procs);

  void *vec = rs_make_vec();

  struct calc_req req;
  req.precision = REQ_PRECISION;

  size_t block_size = (REQ_RANGE / num_procs - 1) + 1;

  // MPI_Type_create_struct(3,

  for (int i = 1; i < num_procs; i++) {
    size_t maybe_start = (i - 1) * block_size;
    size_t maybe_end = (i) * block_size;
    req.start = maybe_start < REQ_RANGE ? maybe_start : REQ_RANGE;
    req.end = maybe_end < REQ_RANGE ? maybe_end : REQ_RANGE;

    // MPI_Send(&req, 1,
    MPI_Send(&req.start, 1, MPI_INT64_T, i, 0, MPI_COMM_WORLD);
    MPI_Send(&req.end, 1, MPI_INT64_T, i, 0, MPI_COMM_WORLD);
    MPI_Send(&req.precision, 1, MPI_INT32_T, i, 0, MPI_COMM_WORLD);
  }

  for (int i = 1; i < num_procs; i++) {
    uint32_t len = 0;
    MPI_Recv(&len, 1, MPI_INT32_T, MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD,
             NULL);

    // char ser[len];
    char *ser = malloc(len + 2);
    MPI_Recv(ser, len, MPI_INT32_T, MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD,
             NULL);
    rs_add_result(vec, ser);
  }
  rs_sum_results(vec, REQ_PRECISION);
}

void generator(int id, int num_procs) {
  printf("Generator %d checking in\n", id);
  // MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
  // MPI_Comm comm, MPI_Status *status)

  size_t start = 0;
  MPI_Recv(&start, 1, MPI_INT64_T, MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD,
           NULL);
  size_t end = 0;
  MPI_Recv(&end, 1, MPI_INT64_T, MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD,
           NULL);
  int precision = 0;
  MPI_Recv(&precision, 1, MPI_INT32_T, MPI_ANY_SOURCE, MPI_ANY_TAG,
           MPI_COMM_WORLD, NULL);

  printf("proc %d got start %ld, end %ld, precision %d\n", id, start, end,
         precision);

  char *res = rs_calc_to_cstr(precision, start, end);
  // printf("Generated partial result %s\n", res);

  int len = strlen(res) + 1;
  MPI_Send(&len, 1, MPI_INT32_T, 0, 0, MPI_COMM_WORLD);
  MPI_Send(res, len, MPI_CHAR, 0, 0, MPI_COMM_WORLD);
}

int main(int argc, char **argv) {
  int ierr;

  ierr = MPI_Init(&argc, &argv);
  // std::cout << "Program is starting" << std::endl;
  printf("Program is starting\n");

  int my_id, num_procs;
  ierr = MPI_Comm_rank(MPI_COMM_WORLD, &my_id);
  ierr = MPI_Comm_size(MPI_COMM_WORLD, &num_procs);

  MPI_Barrier(MPI_COMM_WORLD);

  if (my_id == 0) {
    collector(num_procs);
    printf("Collector exits");
  } else {
    generator(my_id, num_procs);
  }
  printf("Error is: %d\n", ierr);
  ierr = MPI_Finalize();

  // std::cout << "Process " << my_id << " checking in out of " << num_procs <<
  // "procs" << std::endl; printf("Process %d checking in out of %d procs",
  // my_id, num_procs);
}
