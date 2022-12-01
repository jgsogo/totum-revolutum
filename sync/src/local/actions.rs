// struct LocalToLocal<LHS: LocalFileMetadata, RHS: LocalFileMetadata>;
//
// impl<LHS: LocalFileMetadata, RHS: LocalFileMetadata> ActionsTrait for LocalToLocal<LHS, RHS> {
//     async fn copy_to_rhs<L: FileMetadata, R: FileMetadata>(
//         lhs: L,
//         rhs: R,
//     ) -> anyhow::Result<(L, R)> {
//         todo!()
//     }
//
//     async fn _rename<T: FileMetadata>(metadata: T) -> anyhow::Result<T> {
//         todo!()
//     }
//
//     async fn _remove<T: FileMetadata>(metadata: T) -> anyhow::Result<T> {
//         todo!()
//     }
//
//     async fn move_to_rhs<L: FileMetadata, R: FileMetadata>(
//         lhs: L,
//         rhs: R,
//     ) -> anyhow::Result<(L, R)> {
//         todo!()
//     }
// }
