<script setup>
</script>

<template>
  <div class="row">
    <div class="col-12 col-md-9 mb-1">

      <div class="card mb-3">
        <div class="card-header">
          Featured
        </div>
        <ul class="list-group list-group-flush">

          <li class="list-group-item list-group-item-action"><a href="/posts/1">Title</a></li>

        </ul>
      </div>


      <div class="d-flex justify-content-center">

        <nav aria-label="Page navigation example">
          <ul class="pagination">
            <li class="page-item disabled"><a class="page-link" href="/posts/pages/1/1">上一页</a></li>

            <li class="page-item active "><a class="page-link" href="/posts/pages/1/1">1</a></li>

            <li class="page-item disabled"><a class="page-link" href="/posts/pages/1/2">下一页</a></li>
          </ul>
        </nav>

      </div>
    </div>
    <!--end of col-->
    <div class="col-12 col-md-3 mb-1">
      <div class="card">
        <div class="card-header">
          Category
        </div>
        <ul class="list-group list-group-flush">
          <li class="list-group-item list-group-item-action">
            <div>
              <a href="#">Category1</a>
            </div>
            <div>
              <i class="icon-chevron-right"></i>
            </div>
          </li>
          <li class="list-group-item list-group-item-action">
            <div>
              <a href="#">Category2</a>
            </div>
            <div>
              <i class="icon-chevron-right"></i>
            </div>
          </li>
        </ul>
      </div>

    </div>
    <!--end of col-->
  </div>
  <!--end of row-->
</template>

<style></style>