import 'package:appflowy/features/share_tab/data/models/share_access_level.dart';
import 'package:appflowy/features/shared_section/data/repositories/shared_pages_repository.dart';
import 'package:appflowy/features/shared_section/models/shared_page.dart';
import 'package:appflowy_backend/protobuf/flowy-error/errors.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_result/appflowy_result.dart';

// Move this file to test folder
class LocalSharedPagesRepositoryImpl implements SharedPagesRepository {
  @override
  Future<FlowyResult<SharedPages, FlowyError>> getSharedPages() async {
    final pages = [
      SharedPage(
        view: ViewPB()
          ..id = '1'
          ..name = 'Welcome Page',
        accessLevel: ShareAccessLevel.fullAccess,
      ),
      SharedPage(
        view: ViewPB()
          ..id = '2'
          ..name = 'Project Plan',
        accessLevel: ShareAccessLevel.readAndWrite,
      ),
      SharedPage(
        view: ViewPB()
          ..id = '3'
          ..name = 'Readme',
        accessLevel: ShareAccessLevel.readOnly,
      ),
    ];
    return FlowyResult.success(pages);
  }

  @override
  Future<FlowyResult<void, FlowyError>> leaveSharedPage(String pageId) async {
    return FlowyResult.success(null);
  }
}
